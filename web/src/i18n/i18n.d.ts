import 'i18next';

declare module 'i18next' {
  interface CustomTypeOptions {
    defaultNS: 'translation';
    resources: {
      translation: {
        choosePieces: string;
        clearAll: string;
        missingCells: string;
        missingCells_zero: string;
        missingCells_one: string;
        missingCells_two: string;
        missingCells_few: string;
        missingCells_many: string;
        missingCells_other: string;
        thinking: string;
        noSolution: string;
        foundSolutions: string;
        foundSolutions_zero: string;
        foundSolutions_one: string;
        foundSolutions_two: string;
        foundSolutions_few: string;
        foundSolutions_many: string;
        foundSolutions_other: string;
        sourceCode: string;
        language: string;
      };
    };
  }
}
