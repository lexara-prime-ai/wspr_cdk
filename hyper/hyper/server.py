import asyncio
import csv
import os

import constants

import python_wrapper.python_wrapper


class Server:
    def __init__(self):
        self.write_path = os.path.join(
            constants.CONSTANTS.FILE_PATH, "wspr_spot_data.csv"
        )

    async def write_to_csv(self):
        """
        Args: self.
        return type: ()
        """

        try:
            output = await python_wrapper.python_wrapper.get_wspr_spots("10", "JSON")
            data = output.get_data()

            # Display data that's being fetched for [DEBUG] purposes.
            await self.display_data(data)

            # Write data to csv. -> <wspr_spots.csv>
            write_path = self.write_path
            print("\nWrite path: \n", write_path)

            # Check if directory exists.
            if not os.path.exists(constants.CONSTANTS.FILE_PATH):
                os.makedirs(constants.CONSTANTS.FILE_PATH)

            with open(write_path, mode="w", newline="") as file:
                writer = csv.writer(file)
                writer.writerow(
                    [
                        "ID",
                        "Time",
                        "Band",
                        "RX Sign",
                        "RX Lat",
                        "RX Lon",
                        "RX Loc",
                        "TX Sign",
                        "TX Lat",
                        "TX Lon",
                        "TX Loc",
                        "Distance",
                        "Azimuth",
                        "RX Azimuth",
                        "Frequency",
                        "Power",
                        "SNR",
                        "Drift",
                        "Version",
                        "Code",
                    ]
                )

                for record in data:
                    writer.writerow(
                        [
                            record["id"],
                            record["time"],
                            record["band"],
                            record["rx_sign"],
                            record["rx_lat"],
                            record["rx_lon"],
                            record["rx_loc"],
                            record["tx_sign"],
                            record["tx_lat"],
                            record["tx_lon"],
                            record["tx_loc"],
                            record["distance"],
                            record["azimuth"],
                            record["rx_azimuth"],
                            record["frequency"],
                            record["power"],
                            record["snr"],
                            record["drift"],
                            record["version"],
                            record["code"],
                        ]
                    )
        except Exception as e:
            print("An [ERROR] occurred: ", e)

    async def display_data(self, data):
        """
        Args: self, data -> WsprSpot dict.
        return type: ()
        """
        for record in data:
            id_field = record["id"]
            time_field = record["time"]
            band_field = record["band"]
            rx_sign_field = record["rx_sign"]
            rx_lat_field = record["rx_lat"]
            rx_lon_field = record["rx_lon"]
            rx_loc_field = record["rx_loc"]
            tx_sign_field = record["tx_sign"]
            tx_lat_field = record["tx_lat"]
            tx_lon_field = record["tx_lon"]
            tx_loc_field = record["tx_loc"]
            distance_field = record["distance"]
            azimuth_field = record["azimuth"]
            rx_azimuth_field = record["rx_azimuth"]
            frequency_field = record["frequency"]
            power_field = record["power"]
            snr_field = record["snr"]
            drift_field = record["drift"]
            version_field = record["version"]
            code_field = record["code"]

            # Verify content.
            print("\nFetching [ROW] >\n")

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

    async def __call__(self):
        await self.write_to_csv()


server = Server()

if __name__ == "__main__":
    asyncio.run(server())
