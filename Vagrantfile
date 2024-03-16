Vagrant.configure("2") do |config|
  config.vm.box = "debian/bullseye64"
  
  config.vm.provider "virtualbox" do |vb|
    vb.memory = 8192
    vb.cpus = 8
  end

  config.vm.provision "shell", inline: <<-SHELL
    export DEBIAN_FRONTEND=noninteractive

    apt-get update
    apt-get install -y binutils build-essential clang curl git make python3 wget zsh

    su -l vagrant -s "/bin/sh" -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . $HOME/.cargo/env && rustup override set nightly && rustup component add rust-src"

    su -l vagrant -s "/bin/sh" -c "curl -fsSO https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh; chmod 755 install.sh; ./install.sh --unattended"
    sed -i 's/ZSH_THEME=".*"/ZSH_THEME="jonathan"/g' /home/vagrant/.zshrc
    chsh -s /bin/zsh vagrant
    echo "cd /vagrant" >> /home/vagrant/.zshrc

    ln -fs /usr/share/zoneinfo/Europe/Paris /etc/localtime
    echo "Europe/Paris" > /etc/timezone
    dpkg-reconfigure -f noninteractive tzdata
  SHELL
end