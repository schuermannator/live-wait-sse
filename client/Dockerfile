FROM node:9.11.1 as builder
RUN mkdir /usr/src/client
COPY . /usr/src/client
ENV PATH /usr/src/client/node_modules/.bin:$PATH
WORKDIR /usr/src/client
RUN npm install && npm run build

FROM nginx:1.13.12-alpine
COPY --from=builder /usr/src/client/dist/index.html /usr/share/nginx/html/index.html
COPY --from=builder /usr/src/client/dist/bundle.js /usr/share/nginx/html/bundle.js
COPY nginx.vh.default.conf /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 8080
#ENTRYPOINT ["./server"]
CMD ["nginx", "-g", "daemon off;"]
