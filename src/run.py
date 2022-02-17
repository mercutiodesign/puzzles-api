import argparse

from main import app


def main():
    parser = argparse.ArgumentParser(
        description="Sanic test server",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("--host", dest="host", default="127.0.0.1")
    parser.add_argument("--port", dest="port", type=int, default=8002)
    parser.add_argument("--no-reload", dest="no_reload", action="store_true")
    parser.add_argument(
        "--shell",
        dest="shell",
        action="store_true",
        help="Launch an ipython shell after server start",
    )
    parser.add_argument(
        "--shell-point",
        dest="shell_point",
        default="after_server_start",
        help="before_server_start | after_server_start",
    )
    args = parser.parse_args()

    if not args.no_reload:
        import hupper

        reloader = hupper.start_reloader("run.main")
        reloader.watch_files([])

    if args.shell:

        @app.listener(args.shell_point)
        async def launch_shell(app, loop):
            import IPython

            IPython.embed()

    app.run(host=args.host, port=args.port)


if __name__ == "__main__":
    main()
