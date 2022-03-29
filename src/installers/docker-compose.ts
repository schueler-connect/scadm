import chalk from 'chalk';
import { exec } from 'child_process';
import ora from 'ora';

export function execAsync(command: string) {
  return new Promise((resolve, reject) => {
    exec(command, (error, stdout, _stderr) => {
      if (error) reject(error);
      resolve(stdout);
    });
  });
}

export default async function installCompose() {
  const spinner = ora({
    prefixText: chalk`{magenta installer} {blue info}`,
    color: 'cyan',
  }).start(chalk`{cyan docker-compose} wird installiert`);

  try {
		await execAsync('echo "curl -L \"https://github.com/docker/compose/releases/download/1.29.2/docker-compose-\$(uname -s)-\$(uname -m)\" -o /usr/local/bin/docker-compose" | sh');
  } catch (e) {
    spinner.fail();
    throw e;
  }

  spinner.prefixText = chalk`{magenta installer} {green success}`;
  spinner.succeed(chalk`{cyan docker-compose} wurde installiert`);
}
