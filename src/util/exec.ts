import { exec } from "child_process";

export function execAsync(command: string) {
  return new Promise((resolve, reject) => {
    exec(command, (error, stdout, _stderr) => {
      if (error) reject(error);
      resolve(stdout);
    });
  });
}
