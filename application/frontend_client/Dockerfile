FROM node:17-alpine3.15 AS builder

WORKDIR /app
COPY package.json ./

RUN npm install --save --legacy-peer-deps
COPY . .
RUN npm run build

FROM nginx:alpine
WORKDIR /usr/share/nginx/html
RUN rm -rf *
COPY --from=builder /app/build .
ENTRYPOINT ["nginx", "-g", "daemon off;"]