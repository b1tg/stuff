#!/bin/bash

wget https://raw.hellogithub.com/hosts -O /tmp/github_hosts.txt -q


sudo sed -i "s/.*github.*//g" /etc/hosts
sudo sed -i "s/.*Update time.*//g" /etc/hosts


sudo sh -c 'cat /tmp/github_hosts.txt |grep -v "#" |grep "github" >> /etc/hosts'
sudo sh -c 'cat /tmp/github_hosts.txt |grep "Update time" >> /etc/hosts'


cat /tmp/github_hosts.txt |grep "Update time"
