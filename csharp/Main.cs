using Microsoft.AspNet.Builder;
using System.Text;

public class Startup
{
    public void Configure(IApplicationBuilder app)
    {
        app.Run(async context => {
            context.Response.ContentType = "text/plain";
            string body = "Hello world!";
            byte[] data = Encoding.UTF8.GetBytes(body);
            await context.Response.Body.WriteAsync(data, 0, data.Length);
        });
    }
}
