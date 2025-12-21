FROM debian:bullseye-slim

COPY target/release/ms-sql-lab ./back

CMD ["./back"]
