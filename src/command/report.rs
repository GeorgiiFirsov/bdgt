use std::collections::HashMap;
use std::fmt::Write;

use libbdgt::storage::{Account, Plan, Id, Category};
use libbdgt::datetime::Timestamp;

use itertools::Itertools;

use super::command::{Command, CommandInternal};
use crate::error::{Result, Error};
use crate::console::WritePaged;
use crate::datetime;
use crate::binding;
use crate::console;
use crate::errors;
use crate::misc;


/// Time interval [start, end).
type Interval = (Timestamp, Timestamp);


/// Type of table, that contains a report data.
type ReportTable = prettytable::Table;


/// Type of report, that will be printed.
type PrintableReport = (String, ReportTable);


/// Target for a report.
enum ReportTarget {
    /// Report is built for an account. If none specified, all accounts are used.
    Account(Option<Id>),

    /// Report is built for a category.
    Category(()), // For now, reports for single category is not supported

    /// Report is built for a plan. If none specified, all plans are used.
    Plan(Option<Id>),
}


/// Structure with command parameters.
pub(crate) struct Parameters {
    /// Build report for a whole period of time.
    epoch: bool,

    /// Year to build report for.
    year: i32,

    /// Month to build report for.
    month: i32,

    /// Report target
    target: ReportTarget,
}


/// Command for showing application's info.
pub(crate) struct Report;


impl Command for Report {
    const VERB: &'static str = "report";

    const ABOUT: &'static str = "Build a report";

    fn add_args(command: clap::Command) -> clap::Command {
        command
            .arg(
                clap::arg!(-e --epoch "build report for he whole period of time")
                    .conflicts_with_all(["month", "year"])
            )
            .arg(
                clap::arg!(-m --month [MONTH] "month to build report for (defaults to current month)")
                    .conflicts_with("epoch")
                    .default_value("0")
                    .value_parser(clap::value_parser!(i32).range(-1..=12))
                    .allow_negative_numbers(true)
                    .long_help(misc::multiline!(
                        "Possible values for MONTH parameter: [-1 .. 12].",
                        " - Positive values denote number of month in year",
                        " - Zero denotes the current month",
                        " - Negative one denotes the previous month",
                    ))
            )
            .arg(
                clap::arg!(-y --year [YEAR] "year to build report for (defaults to current year)")
                    .conflicts_with("epoch")
                    .default_value("0")
                    .value_parser(clap::value_parser!(i32).range(-1..))
                    .allow_negative_numbers(true)
                    .long_help(misc::multiline!(
                        "Possible values for YEAR parameter: -1, 0 or just a year.",
                        " - Zero denotes the current year",
                        " - Negative one denotes the previous year",
                    ))
            )
            .arg(
                clap::arg!(-a --account <ACCOUNT> "build report for specified account")
                    .value_parser(clap::value_parser!(usize))
                    .conflicts_with_all(["accounts", "categories", "plan", "plans"])
            )
            .arg(
                clap::arg!(--accounts "build report for all accounts (this is default option)")
                    .conflicts_with_all(["account", "categories", "plan", "plans"])
            )
            .arg(
                clap::arg!(--categories "build report for all categories")
                    .conflicts_with_all(["account", "accounts", "plan", "plans"])
            )
            .arg(
                clap::arg!(-p --plan <PLAN> "build report for specified plan")
                    .value_parser(clap::value_parser!(usize))
                    .conflicts_with_all(["accounts", "categories", "plans"])
            )
            .arg(
                clap::arg!(--plans "build report for all plans")
                    .conflicts_with_all(["account", "categories", "plan"])
            )
    }

    fn invoke(matches: &clap::ArgMatches) -> Result<()> {
        let parameters = Self::parse_args(matches)?;
        let budget = binding::open_budget()?;
        let interval = Self::time_interval(&parameters)?;

        //
        // Build reports for the specified entities and time interval
        //

        let reports = match parameters.target {
            ReportTarget::Account(account) => {
                Self::build_accounts_report(budget, interval, account)?
            },
            ReportTarget::Category(_) => {
                Self::build_categories_report(budget, interval)?
            },
            ReportTarget::Plan(plan) => {
                Self::build_plans_report(budget, interval, plan)?
            },
        };

        //
        // Print all reports using pager
        //

        let mut pager = minus::Pager::new();

        for (preamble, table) in reports {
            preamble.write_paged(&mut pager)?;
            table.write_paged(&mut pager)?;

            pager.write_str("\n")?;
        }

        minus::page_all(pager)?;

        Ok(())
    }
}


impl CommandInternal for Report {
    type ParsedArgs = Parameters;

    fn parse_args(matches: &clap::ArgMatches) -> Result<Self::ParsedArgs> {
        let epoch = Self::get_one(matches, "epoch")?;
        let month = Self::get_one(matches, "month")?;
        let year = Self::get_one(matches, "year")?;

        let target = Self::get_target(matches)?;

        Ok(Parameters { 
            epoch: epoch, 
            year: year, 
            month: month,
            target: target
        })
    }
}


impl Report {
    fn get_target(matches: &clap::ArgMatches) -> Result<ReportTarget> {
        if Self::get_one(matches, "accounts")? {
            return Ok(ReportTarget::Account(None));
        }

        if let Some(account) = Self::get_one_opt(matches, "account") {
            return Ok(ReportTarget::Account(Some(account)));
        }

        if Self::get_one(matches, "categories")? {
            return Ok(ReportTarget::Category(()));
        }

        if Self::get_one(matches, "plans")? {
            return Ok(ReportTarget::Plan(None));
        }

        if let Some(plan) = Self::get_one_opt(matches, "plan") {
            return Ok(ReportTarget::Plan(plan));
        }

        //
        // By default, report is built for all accounts
        //

        Ok(ReportTarget::Account(None))
    }
}


impl Report {
    fn build_accounts_report(budget: binding::Budget, interval: Option<Interval>, account: Option<Id>) -> Result<Vec<PrintableReport>> {
        //
        // Query for account(s) data
        //

        let accounts = match account {
            Some(account) => vec![budget.account(account)?],
            None => budget.accounts()?
        };

        //
        // Get categories and convert them into HashMap
        //

        let categories = budget.categories()?;
        let categories = categories
            .into_iter()
            .map(|category| (category.id.unwrap(), category))
            .collect();

        //
        // Let's build reports here!
        //

        let mut reports = Vec::new();
        for account in accounts {
            reports.push(Self::internal_build_account_report(&budget, &interval, &account, &categories)?)
        }

        Ok(reports)
    }

    fn internal_build_account_report(budget: &binding::Budget, interval: &Option<Interval>, account: &Account, categories: &HashMap<Id, Category>) -> Result<PrintableReport> {
        let preamble = format!("Account: {}\nIdentifier: {}\nCurrent balance: {}\n",
            account.name, uuid::Uuid::from_bytes(account.id.unwrap()), console::colorize_amount(account.balance));
        
        //
        // Query for transactions, that correspond to the account
        //

        let transactions = match interval {
            Some((start_timestamp, end_timestamp)) => {
                budget.transactions_of_between(account.id.unwrap(), *start_timestamp, *end_timestamp)?
            },
            None => {
                budget.transactions_of(account.id.unwrap())?
            }
        };

        //
        // Now let's build a report
        //

        let mut table = Self::create_report_table(
            prettytable::row!["Description", "Amount", "Date and time", "Category"]);

        for transaction in transactions {
            table.add_row(prettytable::Row::new(vec![
                prettytable::cell!(transaction.description),
                prettytable::cell!(r -> console::colorize_amount(transaction.amount)),
                prettytable::cell!(transaction.timestamp.to_rfc2822()),
                prettytable::cell!(categories.get(&transaction.category_id).unwrap().name)
            ]));
        }

        Ok((preamble, table))
    }
}


impl Report {
    fn build_categories_report(budget: binding::Budget, interval: Option<Interval>) -> Result<Vec<PrintableReport>> {
        //
        // Query transactions
        //

        let transactions = match interval {
            Some((start_timestamp, end_timestamp)) => {
                budget.transactions_between(start_timestamp, end_timestamp)?
            },
            None => {
                budget.transactions()?
            }
        };

        if transactions.is_empty() {
            return Ok(Vec::new());
        }

        //
        // Query all categories and trasform them into a hash table
        //

        let categories = budget.categories()?;
        let categories: HashMap<_, _> = categories
            .into_iter()
            .map(|category| (category.id.unwrap(), category))
            .collect();

        //
        // Now let's build a report
        //

        let report: Vec<_> = transactions
            .into_iter()
            .sorted_unstable_by_key(|transaction| transaction.category_id)
            .group_by(|transaction| transaction.category_id)
            .into_iter()
            .map(|(category, group)| {
                let category = categories.get(&category).unwrap();
                let total = group
                    .fold(0isize, |accumulator, transaction| accumulator + transaction.amount);

                (category.category_type, category.name.to_owned(), total)
            })
            .sorted_by_key(|(category_type, _, total)| (*category_type, *total))
            .collect();

        //
        // Put all data into a table
        //

        let mut table = Self::create_report_table(
            prettytable::row!["Category", "Total amount"]);

        for (_, name, total) in report {
            table.add_row(prettytable::Row::new(vec![
                prettytable::cell!(name),
                prettytable::cell!(r -> console::colorize_amount(total))
            ]));
        }

        Ok(vec![("".to_string(), table)])
    }
}


impl Report {
    fn build_plans_report(budget: binding::Budget, interval: Option<Interval>, plan: Option<Id>) -> Result<Vec<PrintableReport>> {
        //
        // Query for plan(s) data
        //

        let plans = match plan {
            Some(plan) => vec![budget.plan(plan)?],
            None => budget.plans()?
        };

        //
        // Now we are ready to build report (it is summarized )
        //

        let mut table = Self::create_report_table(
            prettytable::row!["Plan", "Category", "Spent", "Remains"]);

        for plan in plans {
            let (category, spent) = Self::internal_build_plan_report(&budget, &interval, &plan)?;

            table.add_row(prettytable::Row::new(vec![
                prettytable::cell!(plan.name),
                prettytable::cell!(category.name),
                prettytable::cell!(r -> spent),
                prettytable::cell!(r -> console::colorize_amount(plan.amount_limit - spent))
            ]));
        }

        Ok(vec![("".to_string(), table)])
    }

    fn internal_build_plan_report(budget: &binding::Budget, interval: &Option<Interval>, plan: &Plan) -> Result<(Category, isize)> {
        //
        // Query for category and transactions that correspond to the plan
        //

        let category = budget.category(plan.category_id)?;
        let transactions = match interval {
            Some((start_timestamp, end_timestamp)) => {
                budget.transactions_with_between(plan.category_id, *start_timestamp, *end_timestamp)?
            },
            None => {
                budget.transactions_with(plan.category_id)?
            }
        };

        //
        // Calculate spent amount of money
        //

        let spent = transactions
            .into_iter()
            .fold(0isize, |accumulator, transaction| accumulator + transaction.amount)
            .abs();

        Ok((category, spent))
    }
}


impl Report {
    fn time_interval(parameters: &Parameters) -> Result<Option<Interval>> {
        if parameters.epoch {
            return Ok(None);
        }

        //
        // Time interval parameters are parsed according to the 
        // following table:
        //
        // +------+------+--------+------------------------------------------+
        // | Case | Year |  Month | Result                                   |
        // +------+------+--------+------------------------------------------+
        // |    1 |   -1 | 1 - 12 | Report for specific month, last year     |
        // |    2 |   -1 |      0 | Report for last year                     |
        // |    3 |   -1 |     -1 | Not supported                            |
        // |    4 |    0 | 1 - 12 | Report for specific month, current year  |
        // |    5 |    0 |      0 | Report for current month                 |
        // |    6 |    0 |     -1 | Report for previous month                |
        // |    7 |  any | 1 - 12 | Report for specific month, specific year |
        // |    8 |  any |      0 | Report for specific year                 |
        // |    9 |  any |     -1 | Not supported                            |
        // +------+------+--------+------------------------------------------+
        //
        // Report for current year can be obtained by providing the 
        // year explicitly.
        //

        let (month, year) = (parameters.month, parameters.year);
        
        if month == -1 && year != 0 {
            //
            // Cases 3 and 9
            //

            return Err(Error::from_message(errors::INVALID_INTERVAL));
        }

        let duration = match (month, year) {
            (0, y) if y != 0 => {
                //
                // Case 2 and case 8
                //

                datetime::Duration::Year(1)
            },
            _ => {
                //
                // Rest cases
                //

                datetime::Duration::Month(1)
            }
        };

        let start = datetime::make_date(
            datetime::absolute_year(year), 
            datetime::absolute_month(month), 
            1)?;

        let end = datetime::advance_date(&start, duration)?;

        Ok(Some((start, end)))
    }

    fn create_report_table(titles: prettytable::Row) -> ReportTable {
        use prettytable::format;

        let format = format::FormatBuilder::new()
            .column_separator('│')
            .borders('│')
            .separator(
                format::LinePosition::Top, 
                format::LineSeparator::new('─', '┬', '┌', '┐')
            )
            .separator(
                format::LinePosition::Title, 
                format::LineSeparator::new('─', '┼', '├', '┤')
            )
            .separator(
                format::LinePosition::Bottom, 
                format::LineSeparator::new('─', '┴', '└', '┘')
            )
            .padding(1, 1)
            .build();

        let mut table = ReportTable::new();
        table.set_format(format);
        table.set_titles(titles);

        table
    }
}
