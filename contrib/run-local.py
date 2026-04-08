#!/usr/bin/env python
import os
import sys
import subprocess
import re
from pathlib import Path


PROJECT_DIR = Path(__file__).resolve().parent.parent
ENV_FILE = PROJECT_DIR.joinpath(".env")


def load_env() -> dict[str, str]:
    print(" [i] Loading .env ...")
    env = dict(**os.environ)
    if ENV_FILE.is_file():
        with open(ENV_FILE, "rt") as env_file:
            for line in env_file:
                # remove whitespace
                line = line.strip()

                # skip empty
                if len(line) == 0:
                    continue

                # skip comments
                if line.startswith("#"):
                    continue

                # parse name and value
                name, val = [part.strip() for part in line.split("=", 1)]

                # don't overwrite existing env vars
                if name in env:
                    continue

                # assign to dict
                env[name] = val

                print(f"     * {name}")

    return env


def get_service_ip(container_name: str) -> str:
    # fetch ip from docker
    process_out = (
        subprocess.check_output(
            [
                "docker",
                "inspect",
                "-f",
                "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}",
                container_name,
            ]
        )
        .decode()
        .strip()
    )

    # check validity
    if not re.match(r"^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$", process_out):
        raise Exception(
            f"couldn't get ip for container {container_name} got: {process_out}"
        )

    return process_out


def inject_services(env: dict[str, str]) -> dict[str, str]:
    print(" [i] Injecting services ...")
    new_env = env.copy()

    rabbitmq_address = f"amqp://{get_service_ip('tulpje-rabbitmq-1')}:5672"
    new_env["RABBITMQ_ADDRESS"] = rabbitmq_address

    discord_proxy_url = f"{get_service_ip('tulpje-nirn-proxy-1')}:8080"
    new_env["DISCORD_PROXY"] = discord_proxy_url

    gateway_queue_url = f"http://{get_service_ip('tulpje-gateway_queue-1')}:80"
    new_env["DISCORD_GATEWAY_QUEUE"] = gateway_queue_url

    redis_url = f"redis://{get_service_ip('tulpje-valkey-1')}:6379"
    new_env["REDIS_URL"] = redis_url

    database_url = f"postgres://{env['POSTGRES_USER']}:{env['POSTGRES_PASSWORD']}@{get_service_ip('tulpje-postgres-1')}/{env['POSTGRES_DB']}"
    new_env["DATABASE_URL"] = database_url

    print(f"     * Redis        : {redis_url}")
    print(f"     * RabbitMQ     : {rabbitmq_address}")
    print(f"     * PostgreSQL   : {database_url}")
    print(f"     * API Proxy    : {discord_proxy_url}")
    print(f"     * Gateway Queue: {gateway_queue_url}")

    return new_env


def main(args) -> None:
    env = inject_services(load_env())

    print(f" [i] Starting: {' '.join(args[1:])} ...")
    os.execvpe(args[1], args[1:], env)


if __name__ == "__main__":
    main(sys.argv)
