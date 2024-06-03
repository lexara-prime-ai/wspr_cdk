from tableauhyperapi import HyperProcess, Connection, Telemetry, CreateMode, \
    TableDefinition, TableName, SqlType, Inserter

with HyperProcess(Telemetry.SEND_USAGE_DATA_TO_TABLEAU,
                  parameters={"default_database_version": "2"}) as hyper:
    with Connection(hyper.endpoint, 'TrivialExample.hyper', CreateMode.CREATE_AND_REPLACE) as connection:
        # Create an `Extract` table inside an `Extract` schema
        connection.catalog.create_schema('Extract')
        example_table = TableDefinition(TableName('Extract', 'Extract'), [
            TableDefinition.Column('rowID', SqlType.big_int()),
            TableDefinition.Column('value', SqlType.big_int()),
        ])
        connection.catalog.create_table(example_table)

        # Insert data using the `Inserter` class
        with Inserter(connection, example_table) as inserter:
            for i in range(1, 101):
                inserter.add_row(
                    [i, i]
                )
            inserter.execute()
