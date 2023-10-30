# remove-boiler-plate
Remove boilerplate code from auto-generated Swagger DTOs, adapting them to dotnet 7 convetions:

1. Only consider `.cs` files
2. Keep the existing `using`s - your IDE will prompt you to remove the unnecessary ones
3. Add `using System.Text.Json.Serialization;`
4. Keep the specified namespace - your IDE will prompt you to fix it for all the files in the folder
5. Convert DTO `class`es in `sealed record`s
6. If it exists, keep the `[DataMember]`, converting it to `[JsonPropertyName]`
7. Convert all `long` property types to `int`
8. Mark all the properties as nullable - eg `int` => `int?`
9. All the code non specified in the previous points will be removed - eg comments
