
export type Optional<T> = T | undefined;

export class Ptr {
  /**
    * Frees the pointer
    * @returns {Promise<void>}
    */
  free(): Promise<void>;
}

export class Address extends Ptr {

  /**
  * @param {string} s
  * @returns {Promise<Address>}
  */
  static from_string(s: string): Promise<Address>;

  /**
  * @returns {Promise<Address>}
  */
  static to_string(): Promise<string>;
}
