import 'i18next';

declare module 'i18next' {
  interface CustomTypeOptions {
    defaultNS: 'translation';
    resources: {
      translation: {
        choosePieces: string;
        clearAll: string;
        missingCells: string;
        missingCells_one: string;
        missingCells_other: string;
        thinking: string;
        noSolution: string;
        foundSolutions: string;
        foundSolutions_one: string;
        foundSolutions_other: string;
        sourceCode: string;
        language: string;
      };
    };
  }
}
