import asyncio

import python_wrapper.python_wrapper


class Server:
    async def main(self):
        output = await python_wrapper.python_wrapper.get_wspr_spots("10", "JSON")
        data = output.get_data()
        for record in data:
            id_field = record['id']
            time_field = record['time']
            band_field = record['band']
            rx_sign_field = record['rx_sign']
            rx_lat_field = record['rx_lat']
            rx_lon_field = record['rx_lon']
            rx_loc_field = record['rx_loc']
            tx_sign_field = record['tx_sign']
            tx_lat_field = record['tx_lat']
            tx_lon_field = record['tx_lon']
            tx_loc_field = record['tx_loc']
            distance_field = record['distance']
            azimuth_field = record['azimuth']
            rx_azimuth_field = record['rx_azimuth']
            frequency_field = record['frequency']
            power_field = record['power']
            snr_field = record['snr']
            drift_field = record['drift']
            version_field = record['version']
            code_field = record['code']

            # Verify content.
            print("ID:", id_field)
            print("Time:", time_field)
            print("Band:", band_field)
            print("RX Sign:", rx_sign_field)
            print("RX Lat:", rx_lat_field)
            print("RX Lon:", rx_lon_field)
            print("RX Loc:", rx_loc_field)
            print("TX Sign:", tx_sign_field)
            print("TX Lat:", tx_lat_field)
            print("TX Lon:", tx_lon_field)
            print("TX Loc:", tx_loc_field)
            print("Distance:", distance_field)
            print("Azimuth:", azimuth_field)
            print("RX Azimuth:", rx_azimuth_field)
            print("Frequency:", frequency_field)
            print("Power:", power_field)
            print("SNR:", snr_field)
            print("Drift:", drift_field)
            print("Version:", version_field)
            print("Code:", code_field)
            print("---")

    async def __call__(self):
        await self.main()


server = Server()

if __name__ == "__main__":
    asyncio.run(server())
