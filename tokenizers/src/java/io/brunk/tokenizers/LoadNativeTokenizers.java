package io.brunk.tokenizers;

public class LoadNativeTokenizers {
    static {
        try {
            NativeLoader.load("tokenizers");
        } catch (RuntimeException e) {
            throw e;
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }
}
