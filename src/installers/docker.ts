import chalk from 'chalk';
import { exec } from 'child_process';
import ora from 'ora';
import { which } from 'shelljs';
import createLogger from '../util/logger';

export function execAsync(command: string) {
	return new Promise((resolve, reject) => {
		exec(command, (error, stdout, _stderr) => {
			if (error) reject(error);
			resolve(stdout);
		})
	});
}

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
    await execAsync('dnf -y install dnf-plugins-core');
    await execAsync(
      'dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo'
    );
    spinner.text = 'docker-ce';
    await execAsync('dnf -y install docker-ce');
    spinner.text = 'docker-ce-cli';
    await execAsync('dnf -y install docker-ce-cli');
    spinner.text = 'containerd.io';
    await execAsync('dnf -y install containerd.io');
    spinner.text = chalk`{cyan docker} wird installiert`;
    await execAsync('systemctl start docker');
  } else if (which('apt-get')) {
    spinner.stop();
    logger.info(
      chalk`Package-manager {cyan apt} erkannt. Packages für Ubuntu/Debian werden installiert.`
    );
    spinner.start();
    spinner.text = 'apt-get update';
    await execAsync('apt-get update');
    spinner.text = 'ca-certificates';
    await execAsync('apt-get install -y ca-certificates');
    spinner.text = 'curl';
    await execAsync('apt-get install -y curl');
    spinner.text = 'gnupg';
    await execAsync('apt-get install -y gnupg');
    spinner.text = 'lsb-release';
    await execAsync('apt-get install -y lsb-release');
    spinner.text = 'repo (1/2)';
    await execAsync(
      'curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg'
    );
    spinner.text = 'repo (2/2)';
    await execAsync(
      'echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
        $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null'
    );
    spinner.text = 'apt update';
		await execAsync('apt-get update');
		spinner.text = 'docker-ce';
		await execAsync('apt-get install -y docker-ce');
		spinner.text = 'docker-ce-cli';
		await execAsync('apt-get install -y docker-ce-cli');
		spinner.text = 'containerd.io';
		await execAsync('apt-get install -y containerd.io');
    spinner.text = chalk`{cyan docker} wird installiert`;
  } else if (which('yum')) {
    spinner.stop();
    logger.info(
      chalk`Package-manager {cyan yum} erkannt. Packages für CentOS werden installiert.`
    );
    spinner.start();
    spinner.text = 'yum-utils';
    await execAsync('yum install -y yum-utils');
    await execAsync(
      'yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo'
    );
    spinner.text = 'docker-ce';
    await execAsync('yum install -y docker-ce');
    spinner.text = 'docker-ce-cli';
    await execAsync('yum install -y docker-ce-cli');
    spinner.text = 'containerd.io';
    await execAsync('yum install -y containerd.io');
    spinner.text = chalk`{cyan docker} wird installiert`;
    await execAsync('systemctl start docker');
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
