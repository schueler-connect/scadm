import { which } from "shelljs";

export function getPackageManager(): 'apt' | 'yum' | 'brew' | 'install-module' | undefined {
	if (which('apt')) return 'apt';
	if (which('yum')) return 'yum';
	if (which('brew')) return 'brew';
	if (which('install-module')) return 'install-module';
}
