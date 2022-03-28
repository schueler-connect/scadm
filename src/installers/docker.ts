import chalk from 'chalk';
import { execSync } from 'child_process';
import ora from 'ora';
import { which } from 'shelljs';
import createLogger from '../util/logger';

export default async function installDocker() {
  const logger = createLogger('installer', 'magenta');
  const spinner = ora({
    prefixText: chalk`{magenta installer} {blue info}`,
    color: 'cyan',
  }).start(chalk`{cyan docker} wird installiert`);

  if (which('dnf')) {
    spinner.stop();
    logger.info(
      chalk`Package-manager {cyan dnf} erkannt. Packages für Fedora werden installiert.`
    );
    spinner.start();
    spinner.text = 'dnf-plugins-core';
    execSync('dnf -y install dnf-plugins-core');
    execSync(
      'dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo'
    );
    spinner.text = 'docker-ce';
    execSync('dnf -y install docker-ce');
    spinner.text = 'docker-ce-cli';
    execSync('dnf -y install docker-ce-cli');
    spinner.text = 'containerd.io';
    execSync('dnf -y install containerd.io');
    spinner.text = chalk`{cyan docker} wird installiert`;
    execSync('systemctl start docker');
  } else if (which('apt')) {
    spinner.stop();
    logger.info(
      chalk`Package-manager {cyan apt} erkannt. Packages für Ubuntu/Debian werden installiert.`
    );
    spinner.start();
    spinner.text = 'apt update';
    execSync('apt update');
    spinner.text = 'ca-certificates';
    execSync('apt install -y ca-certificates');
    spinner.text = 'curl';
    execSync('apt install -y curl');
    spinner.text = 'gnupg';
    execSync('apt install -y gnupg');
    spinner.text = 'lsb-release';
    execSync('apt install -y lsb-release');
    spinner.text = 'repo (1/2)';
    execSync(
      'curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg'
    );
    spinner.text = 'repo (2/2)';
    execSync(
      'echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
        $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null'
    );
    spinner.text = 'apt update';
		execSync('apt update');
		spinner.text = 'docker-ce';
		execSync('apt install -y docker-ce');
		spinner.text = 'docker-ce-cli';
		execSync('apt install -y docker-ce-cli');
		spinner.text = 'containerd.io';
		execSync('apt install -y containerd.io');
    spinner.text = chalk`{cyan docker} wird installiert`;
  } else if (which('yum')) {
    spinner.stop();
    logger.info(
      chalk`Package-manager {cyan yum} erkannt. Packages für CentOS werden installiert.`
    );
    spinner.start();
    spinner.text = 'yum-utils';
    execSync('yum install -y yum-utils');
    execSync(
      'yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo'
    );
    spinner.text = 'docker-ce';
    execSync('yum install -y docker-ce');
    spinner.text = 'docker-ce-cli';
    execSync('yum install -y docker-ce-cli');
    spinner.text = 'containerd.io';
    execSync('yum install -y containerd.io');
    spinner.text = chalk`{cyan docker} wird installiert`;
    execSync('systemctl start docker');
  } else {
    spinner.fail();
    logger.error(
      chalk`Ihr betriebssystem scheint keinen unterstützten package-manager zu besitzen (unterstützt sind {cyan apt}, {cyan yum} und {cyan dnf})`
    );
    throw new Error();
  }

  spinner.prefixText = chalk`{magenta installer} {green success}`;
  spinner.succeed(chalk`{cyan docker} wurde installiert`);
}
