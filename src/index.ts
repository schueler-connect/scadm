#!/usr/bin/env node
import getPackageVersion from '@jsbits/get-package-version';
import { Command } from 'tauris';
import config from './config';
import header from './header';
import setup from './setup';
import start from './start';
import sysinfo from './sysinfo';

const argv = new Command('scadm', {
  language: {
    'Usage:': 'Nutzung:',
    'Commands:': 'Befehle:',
    'Root Options:': 'Root-Optionen:',
    'Options:': 'Optionen:',
    'Display this help message': 'Diese Hilfsinformationen anzeigen',
  },
})
  .header(header)
  .option('v', {
    alias: 'version',
    description: 'Versionsnummer anzeigen',
    type: 'boolean',
  })
  .rootOption('s', {
    alias: 'silent',
    description: 'Output unterdrücken',
  })
  .command(setup)
  .command(sysinfo)
  .command(start)
  .command(config)
  .demandArgument()
  .parse(process.argv.slice(2), { noPromise: true }) as
  | { [key: string]: any }
  | false;

if (argv) {
  if (argv.v) {
    console.log(`v${getPackageVersion(__dirname)}`);
  }
}
