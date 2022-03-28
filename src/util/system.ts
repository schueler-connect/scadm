import { release, type, userInfo } from 'os';
import tryCatch from './trycatch';

interface SystemInfo {
  arch: 'x86' | 'arm' | null; // null => unsupported
  bits: 32 | 64 | null; // null => unsupported
  username: string;
  os: string;
  os_release: string;
  os_supported: boolean;
}

export async function getSystemInfo(): Promise<SystemInfo> {
  return {
    arch: ['arm', 'arm64'].includes(process.arch)
      ? 'arm'
      : ['x32', 'x64'].includes(process.arch)
      ? 'x86'
      : null,
    bits: ['arm', 'x32'].includes(process.arch)
      ? 32
      : ['x64', 'arm64'].includes(process.arch)
      ? 64
      : null,
    username: userInfo().username,
    os: type(),
    os_release: release(),
    os_supported: ['Linux', 'Darwin'].includes(type()),
  };
}

export async function isSupported(i?: SystemInfo) {
  const info = i || await getSystemInfo();

  return info.arch && info.os_supported;
}
