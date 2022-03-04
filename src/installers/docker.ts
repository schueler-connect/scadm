import ora from "ora";
import { getPackageManager } from "./util";

export default function installDocker() {
  const spinner = ora().start("Docker wird installiert");

  const pm = getPackageManager();
  if (!pm) {
    spinner.fail("Kein package-manager");
    console.log(
      `Bitte installieren sie einen package-manager. Unter MacOS wird "brew" (homebrew) benötigt. Unter Windows wird "Install-Package" benötigt.`
    );
  }

	if (pm === 'apt') {
		
	}
}
