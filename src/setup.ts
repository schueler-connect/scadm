import Enquirer from 'enquirer';
import { Command } from 'tauris';
import header from './header';
import ora from 'ora';
import { which } from 'shelljs';
import createLogger from './util/logger';
import start from './util/time';
import { checkDeps } from './util/dependencies';
import chalk from 'chalk';
import isElevated from 'is-elevated';
import { isSupported } from './util/system';
import installDocker from './installers/docker';
import installCompose from './installers/docker-compose';
import { existsSync } from 'fs';
import { resolve } from 'path';
import { homedir } from 'os';
import askUntilOk from './util/ask-loop';

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
  .handler(async () => {
    const finish = start();
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
      'Diese tool hilft Ihnen, Schüler Connect auf Ihrem server einzurichten. Bitte fahren Sie den computer während der installation nicht herunter.'
    );
    console.log();

    const { ready }: any = await prompt({
      type: 'confirm',
      message: 'Bereit?',
      name: 'ready',
    });
    if (!ready) return finish('cancelled');

    const spinner = ora().start('Suchen nach benötigten tools suchen');
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

    const configPath = '/etc/scadm';
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

    const configData = {
      installLocation:
        (
          (await prompt({
            name: 'location',
            type: 'text',
            message: chalk`In welchem pfad soll der Server installiert werden? {cyan ${resolve(
              homedir(),
              '.scadm/serve'
            )}}`,
          })) as any
        ).location || resolve(homedir(), '.scadm/serve'),
      cloudToken: (
        (await askUntilOk(
          {
            type: 'text',
            name: 'token',
            message: chalk`Cloud-registrierungs-token (dieses erhalten Sie per Anfrage an {cyan mail@schuelerconnect.org}) {red.bold *}`,
          },
          ({ token }: any) => !!token
        )) as any
      ).token,
			authProvider: 'local',
    };

    finish(true);
  });

export default setup;
