import chalk from 'chalk';
import ora from 'ora';
import { Command } from 'tauris';
import { checkDeps, Dependencies } from './util/dependencies';
import createLogger from './util/logger';
import { getSystemInfo, isSupported } from './util/system';

const sysinfo = new Command('sysinfo')
  .describe('Allgemeine Systeminformationen abrufen')
  .handler(async (argv) => {
    const sysinfo = await getSystemInfo();
    const l1 = createLogger('scadm');
    const supported = chalk.green('✔︎');
    const unsupported = chalk.red('✖︎');

    const spinner = ora('Bitte warten').start();
    const dependencies = checkDeps();
    spinner.stop();

    l1.info(chalk.bold('Systeminformationen:'));
    l1.info(
      `CPU:                ${
        sysinfo.arch
          ? `${supported} ${sysinfo.arch} (${sysinfo.bits} bit)`
          : chalk.red(`${unsupported} ${process.arch} (Nicht unterstützt)`)
      }`
    );
    l1.info(
      `Betriebssystem:     ${
        sysinfo.os_supported
          ? `${supported} ${sysinfo.os} (${sysinfo.os_release})`
          : chalk.red(
              `${unsupported} ${sysinfo.os} (${sysinfo.os_release}) (Nicht unterstützt)`
            )
      }`
    );
    for (const name in dependencies) {
      l1.info(
        `${name}:${' '.repeat(19 - name.length)}${
          dependencies[name as keyof Dependencies].available
            ? dependencies[name as keyof Dependencies].supported
              ? `${supported} ${
                  dependencies[name as keyof Dependencies].version
                }`
              : `${unsupported} ${
                  dependencies[name as keyof Dependencies].version
                }`
            : `🚫 Fehlt`
        }`.replace('\n', '')
      );
    }
    console.log();
    if (await isSupported()) l1.info('System unterstützt');
    else l1.info('😧 System nicht unterstützt');
    l1.info(
      `${supported} = Unterstützt\t${unsupported} = Nicht unterstützt\t🚫 = Fehlt`
    );
  });

export default sysinfo;
