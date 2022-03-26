import { performance } from 'perf_hooks';

export default function start() {
  let s = performance.now();
  return (success: boolean | 'cancelled') => {
    console.log(
      `${
        success
          ? success === 'cancelled'
            ? '🚫 Abgebrochen'
            : '✨ Abgeschlossen'
          : '😶 Fehlgeschlagen'
      } in ${(performance.now() - s).toFixed(2)}ms`
    );
  };
}
