import getPackageVersion from '@jsbits/get-package-version';
import { Command } from 'tauris';
import header from './header';
import setup from './setup';
import sysinfo from './sysinfo';

const argv = new Command('scadm', { noDefaultHelpOption: true })
  .header(header)
  .option('h', {
    alias: ['help'],
    description: 'Diese hilfsinformationen anzeigen',
    type: 'boolean',
  })
  .option('v', {
    alias: ['version'],
    description: 'Versionsnummer anzeigen',
    type: 'boolean',
  })
  .command(setup)
  .command(sysinfo)
  .demandArgument()
  .parse(process.argv.slice(2));

if (argv) {
  if (argv.v) {
    console.log(`v${getPackageVersion(__dirname)}`);
  }
}
