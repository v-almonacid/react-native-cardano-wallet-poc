package io.emurgo.rncardanowallet;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;

import android.util.Base64;
import java.util.HashMap;
import java.util.Map;

import androidx.annotation.NonNull;

public class CardanoWalletPocModule extends ReactContextBaseJavaModule {

    private final ReactApplicationContext reactContext;

    public CardanoWalletPocModule(ReactApplicationContext reactContext) {
        super(reactContext);
        this.reactContext = reactContext;
    }

    @Override
    public String getName() {
        return "CardanoWalletPoc";
    }

    @ReactMethod
    public final void addressFromString(String str, Promise promise) {
        Native.I
                .addressFromString(str)
                .map(RPtr::toJs)
                .pour(promise);
    }

}
