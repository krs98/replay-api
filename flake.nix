{
  description = "Rust web dev.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = (import nixpkgs) { inherit system; };
          naersk' = pkgs.callPackage naersk {};
      in {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            cargo-watch
            clippy
            fish
            httpie
            jq
            pgcli
            postgresql
            redis
            rustc
            rust-analyzer
            sqlx-cli
          ];

          shellHook = ''
            mkdir -p .nix-shell

            export NIX_SHELL_DIR=$PWD/.nix-shell
            export PGDATA=$NIX_SHELL_DIR/db
            export REDIS_DATA=$NIX_SHELL_DIR/redis

            export PATH="./scripts/:''${PATH}"
            
            trap \
              "
                pg_ctl -D $PGDATA stop
                redis-cli shutdown

                cd $PGDATA
                rm -rf $NIX_SHELL_DIR
              " \
            EXIT

            if ! test -d $PGDATA 
            then
              pg_ctl initdb -D $PGDATA
            fi

            if ! test -d $REDIS_DATA
            then 
              mkdir -p $REDIS_DATA
              echo "dir $REDIS_DATA" > $REDIS_DATA/redis.conf
              redis-server $REDIS_DATA/redis.conf --daemonize yes 
            fi

            pg_ctl                                                    \
              -D $PGDATA                                              \
              -l $PGDATA/postgres.log                                 \
              -o "-c unix_socket_directories='$PGDATA'"               \
              -o "-c listen_addresses='*'"                            \
              -o "-c log_destination='stderr'"                        \
              -o "-c logging_collector=on"                            \
              -o "-c log_directory='log'"                             \
              -o "-c log_filename='postgresql-%Y-%m-%d_%H%M%S.log'"   \
              -o "-c log_min_messages=info"                           \
              -o "-c log_min_error_statement=info"                    \
              -o "-c log_connections=on"                              \
              start

            echo "
              select 'create database replay' 
              where not exists (select from pg_database where datname = 'replay')\\gexec
            " | psql -h $PGDATA postgres

            fish
          '';
        };
      });

}
