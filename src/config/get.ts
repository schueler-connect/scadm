import chalk from 'chalk';
import chromafi from 'chromafi';
import { existsSync, promises } from 'fs';
import { dirname } from 'path';
import { Command } from 'tauris';
import { Config, configPath } from '../util/config';
import createLogger from '../util/logger';
import tryCatch from '../util/trycatch';

const get = new Command('get')
  .describe('Wert einer Einstellung ablesen')
  .usage('scadm config get [name]')
  .handler(async (argv) => {
    const logger = createLogger('scadm', argv.s);

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

    if (!argv.parameters[0])
      return console.log(chromafi(JSON.stringify(config, null, 2)));
    if (!argv.parameters[0])
      return logger.error('Diese option ist mit keinem Wert belegt.');
    console.log(
      chromafi(
        JSON.stringify(
          argv.parameters[0]
            .split('.')
            .reduce((a: Config, v: string) => a?.[v as keyof Config], config),
          null,
          2
        )
      )
    );
  });

export default get;
