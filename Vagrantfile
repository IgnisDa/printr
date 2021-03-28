# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  # Set the base box used to build the image
  config.vm.box = "bento/ubuntu-20.10"
  # Name of the machine that is created
  config.vm.host_name = "printr-development"
  # Network settings and port forwarding
  config.vm.network "forwarded_port", guest: 8000, host: 8000, host_ip: "0.0.0.0"
  # Gitzer development servers
  config.vm.network "forwarded_port", guest: 8534, host: 8534, host_ip: "0.0.0.0"
  config.vm.network "forwarded_port", guest: 8533, host: 8533, host_ip: "0.0.0.0"
  # This folder is synced to the machine
  config.vm.synced_folder ".", "/home/vagrant/printr"
  # VirtualBox is the default provider, and we specify some specific settings here
  config.vm.provider "virtualbox" do |vb|
    vb.memory = "2048"
    vb.cpus = "2"
  end
  # This script sets up python3, npm, yarn, postgresql etc.
  config.vm.provision :shell, :path => "tools/Vagrant/bootstrap"
  # This script installs each specific application's dependencies
  config.vm.provision :shell, :path => "tools/Vagrant/setup-deps", privileged: false
  # set the API key for the wakatime plugin if it exists in the system
  wakatime_file = File.join(ENV['HOME'],".wakatime.cfg")
  if File.file?(wakatime_file)
    file_data = File.read(wakatime_file)
    $script = <<-SCRIPT
      echo -e "#{file_data}" >> $HOME/.wakatime.cfg
    SCRIPT
    config.vm.provision "shell", inline: $script, privileged: false
  end
  # This script sets some environment variables that are essential to development
  # This is done by adding them to /etc/environment
  config.vm.provision :shell, :path => "tools/shell/env-vars"
end
