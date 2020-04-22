export class Address extends Ptr {

  /**
  * ```
  * @param {string} s
  * @returns {Promise<Address>}
  */
  static from_string(s: string): Promise<Address>;
}
