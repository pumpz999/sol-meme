module.exports = {
  extends: [
    'eslint:recommended',
    'plugin:react/recommended',
    'plugin:@typescript-eslint/recommended',
    'next/core-web-vitals'
  ],
  plugins: ['react', '@typescript-eslint'],
  rules: {
    // Error Prevention
    'no-unused-vars': 'error',
    'no-console': 'warn',
    'no-debugger': 'error',
    
    // Performance
    'react/jsx-no-bind': 'warn',
    'react/memo': 'warn',
    
    // Security
    'no-eval': 'error',
    'no-implied-eval': 'error',
    
    // Best Practices
    'complexity': ['warn', 10],
    'max-depth': ['warn', 4],
    'max-lines-per-function': ['warn', 50],
    
    // TypeScript Specific
    '@typescript-eslint/no-explicit-any': 'error',
    '@typescript-eslint/explicit-function-return-type': 'warn'
  },
  settings: {
    react: {
      version: 'detect'
    }
  }
};
