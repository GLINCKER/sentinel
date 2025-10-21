/**
 * Commitlint Configuration
 * Built by Glincker (A GLINR Product)
 *
 * Enforces Conventional Commits format:
 * <type>(<scope>): <subject>
 *
 * Examples:
 *   feat: add new process manager
 *   fix(ui): resolve dashboard rendering issue
 *   docs: update installation guide
 *   chore(deps): update dependencies
 */

export default {
  extends: ['@commitlint/config-conventional'],

  rules: {
    // Type enum
    'type-enum': [
      2,
      'always',
      [
        'feat',     // New feature
        'fix',      // Bug fix
        'docs',     // Documentation only changes
        'style',    // Code style changes (formatting, missing semi-colons, etc)
        'refactor', // Code change that neither fixes a bug nor adds a feature
        'perf',     // Performance improvements
        'test',     // Adding or updating tests
        'build',    // Changes to build system or external dependencies
        'ci',       // CI configuration changes
        'chore',    // Other changes that don't modify src or test files
        'revert',   // Reverts a previous commit
      ],
    ],

    // Subject case
    'subject-case': [
      2,
      'never',
      ['sentence-case', 'start-case', 'pascal-case', 'upper-case'],
    ],

    // Subject length
    'subject-max-length': [2, 'always', 100],
    'subject-min-length': [2, 'always', 3],

    // Subject empty
    'subject-empty': [2, 'never'],

    // Type empty
    'type-empty': [2, 'never'],

    // Type case
    'type-case': [2, 'always', 'lower-case'],

    // Scope case
    'scope-case': [2, 'always', 'lower-case'],

    // Header max length
    'header-max-length': [2, 'always', 100],

    // Body leading blank
    'body-leading-blank': [1, 'always'],

    // Footer leading blank
    'footer-leading-blank': [1, 'always'],
  },
};
