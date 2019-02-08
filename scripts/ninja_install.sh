#!/bin/sh

if [ "$TRAVIS_OS_NAME" == "linux" ]
then
  wget https://github.com/ninja-build/ninja/releases/download/v1.9.0/ninja-linux.zip
  sudo unzip ninja-linux.zip -d /usr/local/bin/
  sudo chmod +x /usr/local/bin/ninja
  sudo ln -s /usr/local/bin/ninja /usr/bin/ninja;
elif [ "$TRAVIS_OS_NAME" == "osx" ]
then
  wget https://github.com/ninja-build/ninja/releases/download/v1.9.0/ninja-mac.zip
  sudo unzip ninja-mac.zip -d /usr/local/bin/
  sudo chmod +x /usr/local/bin/ninja
  sudo ln -s /usr/local/bin/ninja /usr/bin/ninja;
fi
ninja --version