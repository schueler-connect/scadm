import { z } from 'zod';

const BaseConfig = z.object({
  installLocation: z.string(),
  cloudToken: z.string(),
});

const LocalAuthConfig = z.object({
  authProvider: z.literal('local'),
  authSettings: z.object({
    adminCredentials: z.object({
      username: z.string().min(20),
      password: z.string().min(20),
    }),
    jwtToken: z.string().nonempty(),
  }),
});

const LDAPAuthConfig = z.object({
  authProvider: z.literal('ldap'),
  authSettings: z.object({
    ldapURI: z.string().url(),
    jwtToken: z.string().nonempty(),
  }),
});

const Config = BaseConfig.and(LocalAuthConfig.or(LDAPAuthConfig));
export type Config = z.infer<typeof Config>;

export const configPath = '/etc/scadm/config.json';
