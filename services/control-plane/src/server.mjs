/** Read-only, loopback-only development adapter. NOT an authenticated production API. */
import http from 'node:http';
import { pathToFileURL } from 'node:url';
import { getDemoSnapshot } from '../dist/simulation-service.js';
export function createDevServer() {
  const server=http.createServer(async (request,response)=>{
    const send=(status,body)=>{response.writeHead(status,{'Content-Type':'application/json; charset=utf-8','Cache-Control':'no-store','X-Content-Type-Options':'nosniff'});response.end(JSON.stringify(body));};
    const address=server.address();
    const host=typeof address==='object'&&address?`127.0.0.1:${address.port}`:'';
    if(request.headers.host!==host||request.headers.origin)return send(403,{error:'OriginDenied'});
    if(request.method!=='GET'){request.resume();return send(405,{error:'ReadOnlySimulation'});}
    if((request.url?.length??0)>2048)return send(414,{error:'UrlTooLong'});
    try{
      if(request.url==='/healthz')return send(200,{product:'Benk',status:'ok',mode:'simulation',externalActionsEnabled:false});
      if(request.url==='/v1/demo/snapshot')return send(200,await getDemoSnapshot());
      return send(404,{error:'NotFound'});
    }catch{return send(500,{error:'InternalError'});}
  });
  server.requestTimeout=5000;server.headersTimeout=5000;server.keepAliveTimeout=1000;server.maxHeadersCount=32;server.maxConnections=32;
  return server;
}
if(process.argv[1]&&import.meta.url===pathToFileURL(process.argv[1]).href){
  const port=process.env.BENK_DEV_PORT??'4317';
  if(!/^\d{1,5}$/.test(port)||Number(port)>65535)throw new Error('BENK_DEV_PORT must be 0..65535');
  const server=createDevServer();
  server.on('error',()=>{console.error('Benk development server failed to listen');process.exitCode=1;});
  server.listen(Number(port),'127.0.0.1',()=>console.log(JSON.stringify({product:'Benk',mode:'simulation',address:server.address()})));
  let stopping=false;
  const stop=()=>{if(stopping)return;stopping=true;server.close(()=>process.exit(0));setTimeout(()=>{server.closeAllConnections();process.exit(1);},5000).unref();};
  process.on('SIGINT',stop);process.on('SIGTERM',stop);
}
