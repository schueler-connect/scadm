import chalk from 'chalk';
import ora from 'ora';
import { execAsync } from '../util/exec';

export default async function installCompose() {
  const spinner = ora({
    prefixText: chalk`{magenta installer} {blue info}`,
    color: 'cyan',
  }).start(chalk`{cyan docker-compose} wird installiert`);

  try {
		await execAsync('echo "curl -L \"https://github.com/docker/compose/releases/download/1.29.2/docker-compose-\$(uname -s)-\$(uname -m)\" -o /usr/local/bin/docker-compose" | sh');
		await execAsync('chmod +x /usr/local/bin/docker-compose');
  } catch (e) {
    spinner.fail();
    throw e;
  }

  spinner.prefixText = chalk`{magenta installer} {green success}`;
  spinner.succeed(chalk`{cyan docker-compose} wurde installiert`);
}
