package com.conexus.driver.kernel;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;
import java.util.regex.Pattern;

/**
 * Política de comandos que o driver-core pode executar no sistema.
 *
 * {@code executeCommand} era uma execução arbitrária: qualquer código com uma referência ao kernel
 * rodava qualquer comando. Agora só há uma lista curta de comandos de leitura. Carregar e descarregar
 * módulos NÃO passa por aqui: é feito pelo conexus-driver-manager, que verifica a assinatura
 * (ADR-009 e ADR-014).
 */
public final class KernelCommandPolicy {
    private static final Set<String> READ_ONLY = new HashSet<>(Arrays.asList(
            "dmesg", "zcat", "uname", "cat", "getprop", "lsmod", "sync"));
    private static final Pattern SAFE_ARG = Pattern.compile("^[A-Za-z0-9_./=:,+-]{0,256}$");

    private KernelCommandPolicy() {}

    public static boolean isAllowed(String command, String[] args) {
        if (command == null || !READ_ONLY.contains(command)) {
            return false;
        }
        if (args != null) {
            for (String a : args) {
                if (a == null || !SAFE_ARG.matcher(a).matches() || a.contains("..")) {
                    return false;
                }
            }
        }
        return true;
    }

    /** Nome de módulo de kernel: só letras, dígitos e underscore, até 64 caracteres. */
    public static boolean isValidModuleName(String name) {
        return name != null && name.matches("[A-Za-z0-9_]{1,64}");
    }
}
