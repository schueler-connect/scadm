import Enquirer from "enquirer";
import { Command } from "tauris";
import header from "./header";
import ora from 'ora';
import { which } from 'shelljs';

const { prompt } = Enquirer;

const setup = new Command("setup")
  .describe("Server herunterladen und konfigurieren")
  .usage("scadm setup")
  .handler(async () => {
    console.log("\n" + header + "\n");
    console.log(
      "Diese tool hilft ihnen, Schüler Connect auf ihrem server einzurichten. Bitte fahren sie den computer während der installation nicht herunter.\n"
    );

		const { ready }: any = await prompt({
			type: 'confirm',
			message: 'Bereit?',
			name: 'ready'
		});
		if (!ready) return;

		const dependencyState = {
			docker: false,
			dockerCompose: false,
			git: false
		};
		const spinner = ora().start();

		spinner.text = 'Suche docker...';
		if (which('docker')) dependencyState.docker = true;

		spinner.text = 'Suche docker-compose...';
		if (which('docker-compose')) dependencyState.dockerCompose = true;

		spinner.text = 'Suche git...';
		if (which('git')) dependencyState.git = true;

		spinner.succeed('Benötigte tools überprüfen');

		if (!dependencyState.git) {
			const { installGit }: any = await prompt({
				type: 'confirm',
				name: 'installGit',
				message: 'scadm benötigt git. Git automatisch installieren?'
			});

			if (!installGit) {
				console.log(`Bitte installieren sie git und führen sie dann wieder diesen befehl aus.`);
			}
		}

		if (!dependencyState.docker) {
			const { installDocker }: any = await prompt({
				type: 'confirm',
				name: 'installDocker',
				message: 'scadm benötigt docker. Docker automatisch installieren?'
			});

			if (!installDocker) {
				console.log(`Bitte installieren sie docker und führen sie dann wieder diesen befehl aus.`);
			}
		} else if (!dependencyState.dockerCompose) {
			const { installCompose }: any = await prompt({
				type: 'confirm',
				name: 'installCompose',
				message: 'scadm benötigt docker-compose. Docker-compose automatisch installieren?'
			});

			if (!installCompose) {
				console.log(`Bitte installieren sie docker-compose und führen si dann wieder diesen befehl aus.`);
			}
		}
  });

export default setup;
