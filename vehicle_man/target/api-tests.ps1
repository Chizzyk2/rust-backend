# Get/read vehicle info

Invoke-RestMethod -Uri http://localhost:4000/vehicle -Method Get

# Create/ post vehicle info 

Invoke-RestMethod -Uri http://localhost:4000/vehicle -Method Post

$Params = @{
    Uri = 'http://localhost:4000/vehicle'
    Method = 'Post'
    Body = @{
        manufacturer = 'Tesla'
        model = 'Model Y'
        year = 2024
    } | ConvertTo-Json
    ContentType = 'application/json'
}

Invoke-RestMethod @Params


# using query....

$Params = @{
    Uri = 'http://localhost:4000/vehicle?manufacturer=Tesla&model=Model Y&year=2024'
    Method = 'Post'
}

Invoke-RestMethod @Params


# Customer query

$Params = @{
    Uri = 'http://localhost:4000/vehicle'
    Method = 'Post'
    Body = @{
        manufacturer = 'Tesla'
        model = 'Model Y'
        year = 2024
        first_name = 'Mike'
        last_name = 'Uche'
    } | ConvertTo-Json
    ContentType = 'application/json'
}

Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:4000/vehicle?manufacturer=Tesla&model=Model Y&year=2024&first_name=Mike&last_name=Uche'
    Method = 'Post'
}

Invoke-RestMethod @Params