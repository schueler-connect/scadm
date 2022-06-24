import chalk from 'chalk';
import { promises, existsSync } from 'fs';
import ora from 'ora';
import { dirname, join } from 'path';
import { Command } from 'tauris';
import { configPath } from './util/config';
import createLogger from './util/logger';
import start from './util/time';
import tryCatch from './util/trycatch';

const startCmd = new Command('start')
  .usage('scadm start')
  .describe('Den Server starten')
  .handler(async function (argv) {
    const finish = start();
    const logger = createLogger('scadm', argv.s);
    logger.info('Server wird gestartet');

    if (!existsSync(configPath)) {
      logger.error(
        chalk`Der server ist nicht eingerichtet. Bitte führen sie zuerst {cyan scadm setup} aus.`
      );
      process.exit(1);
    }

    const config = await tryCatch(
      async () => JSON.parse((await promises.readFile(configPath)).toString()),
      (): never => {
        logger.error(
          chalk`Einstellungen konnten nicht gelesen werden. Bitte überprüfen Sie die Datei {green ${configPath}}. Falls die Datei nicht mehr zu retten ist, können Sie den Ordner {green ${dirname(
            configPath
          )}} löschen und ein weiteres mal {cyan scadm setup} ausführen. Bitte beachten Sie, dass dadurch alle Einstellungen verloren gehen`
        );
        process.exit(1);
      }
    );

		const composeFile = join(config.installLocation, 'docker-compose.yaml');
		if (!existsSync(composeFile)) {
			logger.warn('Keine konfiguration gefunden. Falls Sie den Server gerade zum ersten Mal ausführen, ist dies kein Problem.');
			logger.info('Konfiguration wird generiert:');
			const spinner = ora('Konfiguration erstellen').start();
			await this.root().parse(['config', 'apply', '-s'], { noExit: true });
			spinner.succeed();
		}

    finish(true);
  });
export default startCmd;
