import { Command } from 'tauris';
import get from './get';

const config = new Command('config')
  .describe('Konfiguration bearbeiten')
	.demandArgument()
  .usage('scadm config <get|set|apply> [...]')
	.command(get)
	// .command(set);

export default config;
