// Copyright (c) Microsoft. All rights reserved.

namespace DirectMethodSender
{
    using System;
    using System.Collections;
    using System.Globalization;
    using System.IO;
    using System.Net;
    using System.Text;
    using System.Threading;
    using System.Threading.Tasks;
    using Microsoft.Azure.Devices.Client;
    using Microsoft.Azure.Devices.Client.Transport.Mqtt;
    using Microsoft.Azure.Devices.Edge.Util;
    using Microsoft.Extensions.Configuration;

    class Program
    {
        public static int Main() => MainAsync().Result;

        static async Task<int> MainAsync()
        {
            Console.WriteLine($"[{DateTime.UtcNow.ToString("MM/dd/yyyy hh:mm:ss.fff tt", CultureInfo.InvariantCulture)}] Main()");

            IConfiguration configuration = new ConfigurationBuilder()
                .SetBasePath(Directory.GetCurrentDirectory())
                .AddJsonFile("config/appsettings.json", optional: true)
                .AddEnvironmentVariables()
                .Build();

            DumpModuleClientConfiguration();

            TimeSpan dmDelay = configuration.GetValue("DMDelay", TimeSpan.FromSeconds(5));

            string targetModuleId = configuration.GetValue("TargetModuleId", "DirectMethodReceiver");

            // Get deviced id of this device, exposed as a system variable by the iot edge runtime
            string targetDeviceId = configuration.GetValue<string>("IOTEDGE_DEVICEID");
            
            TransportType transportType = configuration.GetValue("ClientTransportType", TransportType.Amqp_Tcp_Only);
            Console.WriteLine($"Using transport {transportType.ToString()}");

            ModuleClient moduleClient = await InitModuleClient(transportType);

            (CancellationTokenSource cts, ManualResetEventSlim completed, Option<object> handler)
                = ShutdownHandler.Init(TimeSpan.FromSeconds(5), null);
            Console.WriteLine($"Call direct method to target device [{targetDeviceId}] and module [{targetModuleId}].");
            await CallDirectMethod(moduleClient, dmDelay, targetDeviceId, targetModuleId, cts).ConfigureAwait(false);
            await moduleClient.CloseAsync();
            completed.Set();
            handler.ForEach(h => GC.KeepAlive(h));
            return 0;
        }

        static async Task<ModuleClient> InitModuleClient(TransportType transportType)
        {
            ITransportSettings[] GetTransportSettings()
            {
                switch (transportType)
                {
                    case TransportType.Mqtt:
                    case TransportType.Mqtt_Tcp_Only:
                    case TransportType.Mqtt_WebSocket_Only:
                        return new ITransportSettings[] { new MqttTransportSettings(transportType) };
                    default:
                        return new ITransportSettings[] { new AmqpTransportSettings(transportType) };
                }
            }
            ITransportSettings[] settings = GetTransportSettings();

            ModuleClient moduleClient = await ModuleClient.CreateFromEnvironmentAsync(settings).ConfigureAwait(false);
            await moduleClient.OpenAsync().ConfigureAwait(false);

            Console.WriteLine("Successfully initialized module client.");
            return moduleClient;
        }

        /// <summary>
        /// Module behavior:
        ///        Call HelloWorld Direct Method every 5 seconds.
        /// </summary>
        /// <param name="moduleClient"></param>
        /// <param name="dmDelay"></param>
        /// <param name="targetModuleId"></param>
        /// <param name="cts"></param>
        /// <param name="targetDeviceId"></param>
        /// <returns></returns>
        static async Task CallDirectMethod(
            ModuleClient moduleClient,
            TimeSpan dmDelay,
            string targetDeviceId,
            string targetModuleId, 
            CancellationTokenSource cts)
        {
            while (!cts.Token.IsCancellationRequested)
            {
                Console.WriteLine($"\t{DateTime.Now.ToLocalTime()}> Calling Direct Method on module.");

                // Create the request
                var request = new MethodRequest("HelloWorldMethod", Encoding.UTF8.GetBytes("{ \"Message\": \"Hello\" }"));

                try
                {
                    //Ignore Exception. Keep trying. 
                    MethodResponse response = await moduleClient.InvokeMethodAsync(targetDeviceId, targetModuleId, request);

                    if (response.Status == (int)HttpStatusCode.OK)
                    {
                        await moduleClient.SendEventAsync("AnyOutput", new Message(Encoding.UTF8.GetBytes("Method Call succeeded.")));
                    }
                }
                catch (Exception e)
                {
                    Console.WriteLine(e);
                }
                

                await Task.Delay(dmDelay, cts.Token).ConfigureAwait(false);
            }
        }
        
        static void DumpModuleClientConfiguration()
        {
            Console.WriteLine("[Configuration for module client]");
            IDictionary environmentVariables = Environment.GetEnvironmentVariables();
            Console.WriteLine($"EdgeHubConnectionString={GetValueFromEnvironment(environmentVariables, "EdgeHubConnectionString")}");
            Console.WriteLine($"IOTEDGE_WORKLOADURI={GetValueFromEnvironment(environmentVariables, "IOTEDGE_WORKLOADURI")}");
            Console.WriteLine($"IOTEDGE_DEVICEID={GetValueFromEnvironment(environmentVariables, "IOTEDGE_DEVICEID")}");
            Console.WriteLine($"IOTEDGE_MODULEID={GetValueFromEnvironment(environmentVariables, "IOTEDGE_MODULEID")}");
            Console.WriteLine($"IOTEDGE_IOTHUBHOSTNAME={GetValueFromEnvironment(environmentVariables, "IOTEDGE_IOTHUBHOSTNAME")}");
            Console.WriteLine($"IOTEDGE_AUTHSCHEME={GetValueFromEnvironment(environmentVariables, "IOTEDGE_AUTHSCHEME")}");
            Console.WriteLine($"IOTEDGE_MODULEGENERATIONID={GetValueFromEnvironment(environmentVariables, "IOTEDGE_MODULEGENERATIONID")}");
            Console.WriteLine($"IOTEDGE_GATEWAYHOSTNAME={GetValueFromEnvironment(environmentVariables, "IOTEDGE_GATEWAYHOSTNAME")}");
        }

        static string GetValueFromEnvironment(IDictionary envVariables, string variableName)
        {
            if (envVariables.Contains((object) variableName))
                return envVariables[(object) variableName].ToString();
            return (string) null;
        }
    }
}
