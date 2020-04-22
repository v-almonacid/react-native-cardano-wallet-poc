package io.emurgo.rncardanowallet;

import java.util.Map;

final class Native {
    static final Native I;

    static {
        I = new Native();
        System.loadLibrary("react_native_cardano_wallet");
        I.initLibrary();
    }

    private Native() { }

    private native void initLibrary();

    // Address
    public final native Result<RPtr> addressFromString(String str);
    public final native Result<String> addressToString(RPtr address);

    public final native void ptrFree(RPtr ptr);
}
