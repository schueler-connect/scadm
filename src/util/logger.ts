import chalk from 'chalk';
import { inspect } from 'util';
import wrap from 'wrap-ansi';

function _log(prefix: string, ...message: any) {
  const text = message
    .map((t: any) =>
      typeof t === 'string' ? t : inspect(t, undefined, Infinity, true)
    )
    .join(' ');
  console.log(
    wrap(text, process.stdout.columns - prefix.length - 2)
      .split('\n')
      .map((l) => prefix + ' ' + l)
      .join('\n')
  );
}

export interface Logger {
  parent?: Logger;
  prefix: string;
  silent: boolean;
  success: (...message: any[]) => void;
  info: (...message: any[]) => void;
  warn: (...message: any[]) => void;
  error: (...message: any[]) => void;
  custom: (prefix: string, ...message: any[]) => void;
  topLevel: (prefix: string, ...message: any[]) => void;
}

function _countParents(logger?: Logger) {
  let i = 0;
  while (logger) {
    i++;
    logger = logger.parent;
  }
  return i;
}

const levelColor = {
  success: chalk.green,
  info: chalk.blue,
  warn: chalk.yellow,
  error: chalk.red,
};

export default function createLogger(name: string): Logger;
export default function createLogger(name: string, silent: boolean): Logger;
export default function createLogger(
  name: string,
  color: keyof typeof chalk
): Logger;
export default function createLogger(
  name: string,
  color: keyof typeof chalk,
  silent: boolean
): Logger;
export default function createLogger(name: string, parent: Logger): Logger;
export default function createLogger(
  name: string,
  parent: Logger,
  silent: boolean
): Logger;
export default function createLogger(
  name: string,
  parent: Logger,
  color: keyof typeof chalk
): Logger;
export default function createLogger(
  name: string,
  parent: Logger,
  color: keyof typeof chalk,
  silent: boolean
): Logger;
export default function createLogger(
  name: string,
  ...args: (keyof typeof chalk | Logger | boolean)[]
): Logger {
  const parent: Logger | undefined = <Logger | undefined>(
    args.find((x) => typeof x === 'object' && x !== null)
  );
  const color: keyof typeof chalk | undefined = <
    keyof typeof chalk | undefined
  >args.find((x) => typeof x === 'string');
  const silent: boolean =
    <boolean | undefined>args.find((x) => typeof x === 'boolean') ||
    parent?.silent ||
    false;

  const pCols = [chalk.green, chalk.blue, chalk.yellow, chalk.magenta];
  const prefix = `${parent ? parent.prefix + ' ' : ''}${(!!color
    ? <any>chalk[color]
    : pCols[_countParents(parent) % pCols.length])(name)}`;

  return {
    silent: silent,
    parent: parent,
    prefix: prefix,
    success: (...message: any[]) =>
      silent
        ? undefined
        : _log(prefix + ` ${levelColor.success('success')}`, ...message),
    info: (...message: any[]) =>
      silent
        ? undefined
        : _log(prefix + ` ${levelColor.info('info')}`, ...message),
    warn: (...message: any[]) =>
      silent
        ? undefined
        : _log(prefix + ` ${levelColor.warn('warn')}`, ...message),
    error: (...message: any[]) =>
      silent
        ? undefined
        : _log(prefix + ` ${levelColor.error('error')}`, ...message),
    custom: (prefix_: string, ...message: any[]) =>
      silent ? undefined : _log(prefix + ' ' + prefix_, ...message),
    topLevel: (prefix_: string, ...message: any[]) =>
      silent ? undefined : _log(prefix_, ...message),
  };
}
