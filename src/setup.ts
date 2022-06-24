import Enquirer from 'enquirer';
import { Command } from 'tauris';
import header from './header';
import ora from 'ora';
import createLogger from './util/logger';
import start from './util/time';
import { checkDeps } from './util/dependencies';
import chalk from 'chalk';
import isElevated from 'is-elevated';
import { isSupported } from './util/system';
import installDocker from './installers/docker';
import installCompose from './installers/docker-compose';
import { existsSync, promises } from 'fs';
import { resolve } from 'path';
import { homedir } from 'os';
import askUntilOk from './util/ask-loop';
import { Config, configPath } from './util/config';
import { nanoid } from 'nanoid';
import { randomFillSync } from 'crypto';

const { prompt } = Enquirer;

async function askToInstall(name: string, onNo?: () => void) {
  const { shouldInstall }: any = await prompt({
    type: 'confirm',
    name: 'shouldInstall',
    message: chalk`scadm benötigt {cyan ${name}}. Dieses tool automatisch installieren?`,
  });

  if (shouldInstall) return true;
  else {
    console.log(
      `Bitte installieren Sie ${name} und führen Sie diesen Befehl dann wieder aus.`
    );
    onNo?.();
    process.exit(1);
  }
}

const setup = new Command('setup')
  .describe('Server herunterladen und konfigurieren')
  .usage('scadm setup')
  .clearRoot('s')
  .handler(async function (argv) {
    const finish = start();
    // Note to self: Silence is deliberatly not implemented here
    const logger = createLogger('scadm');
    if (!(await isSupported()))
      (logger.error('Ihr system is nicht unterstützt.') as any) ||
        finish(false) ||
        process.exit(1);
    if (!(await isElevated()))
      (logger.error('Dieser befehl benötigt root-berichtigungen.') as any) ||
        finish('cancelled') ||
        process.exit(1);

    console.log(
      '\n' +
        header
          .split('\n')
          .map((l) => `           ${l}`)
          .join('\n') +
        '\n'
    );
    logger.info(
      'Dieses tool hilft Ihnen, Schüler Connect auf Ihrem server einzurichten. Bitte fahren Sie den computer während der installation nicht herunter.'
    );
    console.log();

    const { ready }: any = await prompt({
      type: 'confirm',
      message: 'Bereit?',
      name: 'ready',
    });
    if (!ready) return finish('cancelled');

    const spinner = ora().start('Suchen nach benötigten tools');
    const dependencyState = checkDeps();

    spinner.succeed();

    if (!dependencyState.docker.available) {
      await askToInstall('docker', () => finish('cancelled'));
      try {
        await installDocker();
      } catch {
        logger.error(
          'Installation fehlgeschlagen. Bitte installieren Sie docker manuell.'
        );
        finish(false);
      }
    }

    if (!dependencyState['docker-compose'].available) {
      await askToInstall('docker-compose', () => finish('cancelled'));
      try {
        await installCompose();
      } catch {
        logger.error(
          'Installation fehlgeschlagen. Bitte installieren Sie docker-compose manuell.'
        );
        finish(false);
      }
    }

    if (existsSync(configPath)) {
      const { proceed }: any = await prompt({
        name: 'proceed',
        type: 'confirm',
        message:
          'Es scheint bereits konfigurationsdateien für scadm auf Ihrem system zu geben. Diese überschreieben?',
      });

      if (!proceed) {
        // TODO: Verify that existing configs are valid
        finish(true);
        process.exit(0);
      }
    }

    logger.info(
      chalk`Wenn hinter einer frage {cyan (blauer text in klammern)} steht, können Sie die Frage leerlassen, um den Wert in den Klammern zu übernehmen`
    );

    const configData: Config = {
      installLocation:
        (
          (await prompt({
            name: 'location',
            type: 'text',
            message: chalk`In welchem pfad soll der Server installiert werden? {cyan (${resolve(
              homedir(),
              '.scadm/server/'
            )})}`,
          })) as any
        ).location || resolve(homedir(), '.scadm/server/'),
      cloudToken: (
        (await askUntilOk(
          {
            type: 'password',
            name: 'token',
            message: chalk`Cloud-registrierungs-token (dieses erhalten Sie per Anfrage an {magenta info@schuelerconnect.org}) {red.bold *}`,
          },
          ({ token }: any) => !!token && token.length === 48
        )) as any
      ).token,
      authProvider: 'local',
      authSettings: {
        adminCredentials: {
          username: nanoid(32),
          password: nanoid(32),
        },
        jwtToken: randomFillSync(Buffer.alloc(32)).toString('base64'),
      },
    };

    await promises.mkdir(configData.installLocation);
    logger.info('Installationsordner erstellt');
    await promises.writeFile(configPath, JSON.stringify(configData));
    logger.info('Einstellungen gespeichert');

    finish(true);
    logger.success('Der server wird nun gestartet.');

    // Start server by running the CLI's `start` command.
    // This is equivalent to `exec('scadm start');`, except that it doesn't
    // spawn a child process and doesn't require the `scadm` CLI command to
    // actually be installed.
    // TODO: Another idea: provide an alternative to `process.exit()` as an
    //       argument to the handler so that it can be passed up if `noExit`
    //       is set by the caller of `parse()`.
    await this.root().parse(['start'], { noExit: true });
  });

export default setup;
