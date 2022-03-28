import { execSync } from 'child_process';
import { which } from 'shelljs';

export interface DependencyInfo {
  supported: boolean;
  available: boolean;
  version: string;
}

export interface Dependencies {
  docker: DependencyInfo;
  'docker-compose': DependencyInfo;
}

export function checkDeps(): Dependencies {
  return {
    docker: Boolean(which('docker'))
      ? {
          supported: true,
          available: true,
          version: execSync('docker -v').toString().slice(7),
        }
      : { supported: false, available: false, version: '' },
    'docker-compose': Boolean(which('docker-compose'))
      ? {
          supported: true,
          available: true,
          version: execSync('docker-compose -v').toString().slice(15),
        }
      : { supported: false, available: false, version: '' },
  };
}
