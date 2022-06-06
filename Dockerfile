FROM node:16-bullseye AS frontend_builder
WORKDIR /app
COPY frontend .
RUN npm install && npm run build

FROM rust:1.60.0-bullseye AS backend_builder
WORKDIR /app
# RUN apt-get update && apt-get install libssl-dev musl-dev
COPY backend .
RUN cargo build --release

FROM node:16-bullseye
WORKDIR /app
RUN apt-get update && apt-get install ffmpeg redis-server -y
COPY --from=frontend_builder /app/build /app
COPY --from=frontend_builder /app/package.json /app
RUN npm install
COPY --from=backend_builder /app/target/release/backend /app/backend
COPY start.sh .
CMD ["bash", "start.sh"]