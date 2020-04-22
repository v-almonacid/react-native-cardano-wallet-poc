import { NativeModules } from 'react-native';
// import { decode as base64_decode, encode as base64_encode } from 'base-64';

const { CardanoWalletPoc } = NativeModules;

class Ptr {
    static _wrap(ptr, klass) {
        if (ptr === '0') {
            return undefined;
        }
        const obj = Object.create(klass.prototype);
        obj.ptr = ptr;
        return obj;
    }

    static _assertClass(ptr, klass) {
        if (!(ptr instanceof klass)) {
            throw new Error(`expected instance of ${klass.name}`);
        }
        return ptr.ptr;
    }

    constructor() {
        throw new Error("Can't be initialized with constructor");
    }

    /**
    * Frees the pointer
    * @returns {Promise<void>}
    */
    async free() {
        if (!this.ptr) {
            return;
        }
        const ptr = this.ptr;
        this.ptr = null;
        await CardanoWalletPoc.ptrFree(ptr);
    }
}


export class Address extends Ptr {

    /**
    * @param {string} s
    * @returns {Promise<Address>}
    */
    static async from_string(s) {
        const ret = await CardanoWalletPoc.addressFromString(s);
        return Ptr._wrap(ret, Address);
    }

    /**
    * @returns {Promise<string>}
    */
    async to_string() {
        return await CardanoWalletPoc.addressToString(this.ptr);
    }

}
