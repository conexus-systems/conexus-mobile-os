#!/bin/sh
# Gradle start up script for UN*X

# Attempt to set APP_HOME
# Resolve links: $0 may be a link
PRG="$0"
while [ -h "$PRG" ] ; do
    ls=`ls -ld "$PRG"`
    link=`expr "$ls" : '.*-> \(.*\)$'`
    if expr "$link" : '/.*' > /dev/null; then
        PRG="$link"
    else
        PRG=`dirname "$PRG"`/"$link"
    fi
done

SAVED="`pwd`"
cd "`dirname \"$PRG\"`/" >/dev/null
APP_HOME="`pwd -P`"
cd "$SAVED" >/dev/null

APP_NAME="Gradle"
APP_BASE_NAME=`basename "$0"`

DEFAULT_JVM_OPTS='"-Xmx64m" "-Xms64m"'

MAX_FD="maximum"

if [ -n "$JAVA_HOME" ] ; then
    JAVA_EXEC="$JAVA_HOME/bin/java"
else
    JAVA_EXEC="java"
    which java >/dev/null 2>&1 || die "ERROR: JAVA_HOME is not set and no 'java' command could be found in your PATH.
Please set the JAVA_HOME variable in your environment to match the
location of your Java installation."
fi

if [ -n "$GRADLE_OPTS" ] ; then
    JAVA_OPTS="$GRADLE_OPTS"
fi

if [ -n "$JAVA_OPTS" ] ; then
    JAVA_OPTS="$JAVA_OPTS $DEFAULT_JVM_OPTS"
else
    JAVA_OPTS="$DEFAULT_JVM_OPTS"
fi

if [ -n "$CLASSPATH" ] ; then
    CLASSPATH="$CLASSPATH"
fi

if [ -z "$CLASSPATH" ] ; then
    CLASSPATH="$APP_HOME/gradle/wrapper/gradle-wrapper.jar"
fi

exec "$JAVA_EXEC" $JAVA_OPTS -classpath "$CLASSPATH" org.gradle.wrapper.GradleWrapperMain "$@"