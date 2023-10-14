#!/bin/zsh

ARCH=$1

GPGME_SRV=https://gnupg.org/ftp/gcrypt/gpgme
GPGERROR_SRV=https://gnupg.org/ftp/gcrypt/libgpg-error

GPGME_PKG=gpgme-1.22.0.tar.gz
GPGERROR_PKG=libgpg-error-1.47.tar.gz

GPGME_ARCHIVE=gpgme.tar.gz
GPGERROR_ARCHIVE=gpg-error.tar.gz

GPGME_FOLDER=gpgme
GPGERROR_FOLDER=gpgerror

#
# Install wget
#
brew install wget

#
# Download and extract dependencies
#
wget -c $GPGME_SRV/$GPGME_PKG -O $GPGME_ARCHIVE
whet -c $GPGERROR_SRV/$GPGERROR_PKG -O $GPGERROR_ARCHIVE

mkdir $GPGME_FOLDER
mkdir $GPGERROR_FOLDER

tar -xzvf $GPGME_ARCHIVE -C $GPGME_FOLDER  --strip-components 1
tar -xzvf $GPGERROR_ARCHIVE -C $GPGERROR_FOLDER  --strip-components 1

#
# Build deps
#
./build-deps.sh $(pwd)/$GPGME_FOLDER $ARCH
./build-deps.sh $(pwd)/$GPGERROR_FOLDER $ARCH

#
# Hide original installed packages if any and we are done
#
./hide-homebrew-pkg.sh gpgme
./hide-homebrew-pkg.sh gpg-error
