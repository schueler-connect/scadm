import chalk from 'chalk';
import Enquirer from 'enquirer';
import { Logger } from './logger';

const { prompt } = Enquirer;

export default async function askUntilOk(
  question: any,
  predicate: (x: any) => boolean,
  logger?: Logger
): Promise<any> {
  while (true) {
    const r = await prompt(question);
    if (predicate(r)) return r;
    else
      (logger?.error || console.error)(
        chalk`Unerwartete Antwort. Bitte überprüfen Sie Ihre antwort auf tippfehler und lesen sie die Frage noch einmal genau durch.`
      );
  }
}
