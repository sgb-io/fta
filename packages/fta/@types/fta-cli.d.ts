declare module "fta-cli" {
  /**
   * Represents an analyzed file with all its metrics.
   *
   * @property {string} file_name - The name of the file.
   * @property {number} cyclo - The cyclomatic complexity of the file.
   * @property {Object} halstead - The Halstead metrics of the file, a complexity measure.
   * @property {number} halstead.uniq_operators - The number of unique operators.
   * @property {number} halstead.uniq_operands - The number of unique operands.
   * @property {number} halstead.total_operators - The total number of operators.
   * @property {number} halstead.total_operands - The total number of operands.
   * @property {number} halstead.program_length - The total count of operators and operands. (N)
   * @property {number} halstead.vocabulary_size - The total count of unique operators and operands. (n)
   * @property {number} halstead.volume - A measure of the size of the program. V = N * log2(n). (V)
   * @property {number} halstead.difficulty - Quantifies how difficult a program is to write or understand. D = (n1/2) * (N2/n2). (D)
   * @property {number} halstead.effort - An estimation of the amount of work required to write a program. E = D * V.
   * @property {number} halstead.time - An estimation of the time required to write the program. T = E / 18 (seconds).
   * @property {number} halstead.bugs - An estimation of the number of bugs in the program. B = V / 3000.
   * @property {number} line_count - The number of lines in the file.
   * @property {number} fta_score - The FTA score of the file.
   * @property {string} assessment - The assessment of the file.
   */
  export type AnalyzedFile = {
    /**
     * The name of the file.
     *
     * @type {string}
     */
    file_name: string;
    /**
     * The cyclomatic complexity of the file.
     *
     * @type {number}
     */
    cyclo: number;
    /**
     * The Halstead metrics of the file, a complexity measure.
     * For further information see the [docs](https://ftaproject.dev/docs/scoring)
     *
     * @type {Object}
     */
    halstead: {
      /**
       * The number of unique operators.
       *
       * @type {number}
       */
      uniq_operators: number;
      /**
       * The number of unique operands.
       *
       * @type {number}
       */
      uniq_operands: number;
      /**
       * The total number of operators.
       *
       * @type {number}
       */
      total_operators: number;
      /**
       * The total number of operands.
       *
       * @type {number}
       */
      total_operands: number;
      /**
       * The total count of operators and operands.
       *
       * @type {number}
       */
      program_length: number;
      /**
       * The total count of unique operators and operands.
       *
       * @type {number}
       */
      vocabulary_size: number;
      /**
       * A measure of the size of the program. V = N * log2(n).
       *
       * @type {number}
       */
      volume: number;
      /**
       * Quantifies how difficult a program is to write or understand. D = (n1/2) * (N2/n2).
       *
       * @type {number}
       */
      difficulty: number;
      /**
       * An estimation of the amount of work required to write a program. E = D * V.
       *
       * @type {number}
       */
      effort: number;
      /**
       * An estimation of the time required to write the program. T = E / 18 (seconds).
       *
       * @type {number}
       */
      time: number;
      /**
       * An estimation of the number of bugs in the program. B = V / 3000.
       *
       * @type {number}
       */
      bugs: number;
    };
    /**
     * The number of lines in the file.
     *
     * @type {number}
     */
    line_count: number;
    /**
     * The FTA score of the file.
     *
     * @type {number}
     */
    fta_score: number;
    /**
     * The assessment of the file.
     *
     * @type {string}
     */
    assessment: string;
  };

  /**
   * Represents the possible options for the FTA-Analysis.
   *
   * @property {boolean} json
   */
  export type FtaAnalysisOptions = {
    /**
     * Wether the result should be returned in a json or a pretty printed table.
     *
     * @type {boolean}
     */
    json: boolean;
  };

  /**
   * Runs the FTA-Analysis for the given project.
   *
   * @param projectPath - The path to the root of the project to analyze
   * @param options - The options for the analysis
   */
  export function runFta(
    projectPath: string,
    options: FtaAnalysisOptions
  ): string;
}
